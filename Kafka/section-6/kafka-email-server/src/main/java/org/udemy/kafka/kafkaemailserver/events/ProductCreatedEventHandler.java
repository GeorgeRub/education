package org.udemy.kafka.kafkaemailserver.events;

import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpMethod;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.kafka.annotation.KafkaHandler;
import org.springframework.kafka.annotation.KafkaListener;
import org.springframework.stereotype.Component;
import org.springframework.web.client.HttpServerErrorException;
import org.springframework.web.client.ResourceAccessException;
import org.springframework.web.client.RestTemplate;
import org.udemy.kafka.kafkacore.models.ProductCreatedEvent;
import org.udemy.kafka.kafkaemailserver.exceptions.NotRetryableException;
import org.udemy.kafka.kafkaemailserver.exceptions.RetryableException;

@Component
@KafkaListener(topics = "product-created-events-topic")
@Slf4j
public class ProductCreatedEventHandler {

    private RestTemplate restTemplate;

    public ProductCreatedEventHandler(RestTemplate restTemplate) {
        this.restTemplate = restTemplate;
    }

    @KafkaHandler
    public void handler(ProductCreatedEvent productCreatedEvent) {
        log.info("Received an event: {}", productCreatedEvent.getTitle());

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

    }

}
