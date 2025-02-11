package org.udemy.kafka.kafkaemailserver.events;

import lombok.AllArgsConstructor;
import lombok.NoArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.dao.DataIntegrityViolationException;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.kafka.annotation.KafkaHandler;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.kafka.support.KafkaHeaders;
import org.springframework.messaging.handler.annotation.Header;
import org.springframework.messaging.handler.annotation.Payload;
import org.springframework.stereotype.Component;
import org.springframework.transaction.annotation.Transactional;
import org.springframework.web.client.HttpServerErrorException;
import org.springframework.web.client.ResourceAccessException;
import org.springframework.web.client.RestTemplate;
import org.udemy.kafka.kafkacore.models.ProductCreatedEvent;
import org.udemy.kafka.kafkaemailserver.exceptions.NotRetryableException;
import org.udemy.kafka.kafkaemailserver.exceptions.RetryableException;
import org.udemy.kafka.kafkaemailserver.models.ProcessedEvent;
import org.udemy.kafka.kafkaemailserver.repo.ProcessedEventRepo;

@Component
@KafkaListener(topics = "product-created-events-topic")
@Slf4j
@AllArgsConstructor
public class ProductCreatedEventHandler {

    private final RestTemplate restTemplate;
    private final ProcessedEventRepo processedEventRepo;

    @Transactional
    @KafkaHandler
    public void handler(@Payload ProductCreatedEvent productCreatedEvent,
                        @Header(value = "messageId", required = false) String messageId,
                        @Header(KafkaHeaders.RECEIVED_KEY) String messageKey) {
        log.info("-----------------------------------------------------");
        log.info("Received an event: {}", productCreatedEvent.getTitle());
        log.info("Message id: {}", messageId);
        log.info("Message key: {}", messageKey);

        ProcessedEvent event = processedEventRepo.findByMessageId(messageId);
        if(event != null) {
            log.info("Event already processed message id: {}", messageId);
            return;
        }

        try {
            //run request to some microservice
            ResponseEntity<String> response = restTemplate.exchange(
                    "http://localhost:8081/emails", HttpMethod.GET, null, String.class);

            if (response.getStatusCode().value() == HttpStatus.OK.value()) {
                log.info("Email sent" + response.getBody());
            }

        } catch (ResourceAccessException ex) {
            log.error("Error sending email", ex);
            throw new RetryableException(ex);
        } catch (HttpServerErrorException ex) {
            log.error("Error sending email", ex);
            throw new NotRetryableException(ex);
        } catch (Exception ex) {
            log.error("Error sending email", ex);
            throw new NotRetryableException(ex);
        }

        try {
            processedEventRepo.save(ProcessedEvent.builder().messageId(messageId).productId(productCreatedEvent.getProductId()).build());
        } catch (DataIntegrityViolationException ex) {
            log.error("Error saving processed event", ex);
            throw new NotRetryableException(ex);
        }


    }

}
