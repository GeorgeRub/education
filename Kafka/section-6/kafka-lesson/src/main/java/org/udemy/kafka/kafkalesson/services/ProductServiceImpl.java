package org.udemy.kafka.kafkalesson.services;

import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.kafka.core.KafkaTemplate;
import org.springframework.kafka.support.SendResult;
import org.springframework.stereotype.Service;
import org.udemy.kafka.kafkacore.models.ProductCreatedEvent;
import org.udemy.kafka.kafkalesson.controllers.CreateProductRestModel;

import java.util.UUID;
import java.util.concurrent.CompletableFuture;

@Service
@AllArgsConstructor
@Slf4j
public class ProductServiceImpl implements ProductService {

    KafkaTemplate<String, ProductCreatedEvent> kafkaTemplate;

    @Override
    public String createProduct(CreateProductRestModel productRestModel) throws Exception {

        String productId = UUID.randomUUID().toString();

        ProductCreatedEvent productCreatedEvent = ProductCreatedEvent
                .builder()
                .productId(productId)
                .title(productRestModel.title())
                .price(productRestModel.price())
                .quantity(productRestModel.quantity())
                .build();

        SendResult<String, ProductCreatedEvent> result = kafkaTemplate.send("product-created-events-topic", productId, productCreatedEvent).get();
        showInformation(result);
        return productId;
    }

    @Override
    public CompletableFuture<SendResult<String, ProductCreatedEvent>> createProductAsync(CreateProductRestModel productRestModel) {
        log.info("Creating product: {}", productRestModel.title());
        String productId = UUID.randomUUID().toString();

        ProductCreatedEvent productCreatedEvent = ProductCreatedEvent
                .builder()
                .productId(productId)
                .title(productRestModel.title())
                .price(productRestModel.price())
                .quantity(productRestModel.quantity())
                .build();

        CompletableFuture<SendResult<String, ProductCreatedEvent>> future = kafkaTemplate.send("product-created-events-topic", productId, productCreatedEvent);

        future.whenComplete((result, exception) -> {
            if (exception != null) {
                log.error("Error sending message {}", exception.getMessage());
                throw new RuntimeException(exception);
            }
            showInformation(result);
        });

        log.info("returning product id: {}", productId);

        return future;
    }

    private void showInformation(SendResult<String, ProductCreatedEvent> result) {
        log.info("Hello from product service");
        log.info("Send result: {}", result.getRecordMetadata());
        log.info("Send result partition: {}", result.getRecordMetadata().partition());
        log.info("Send result offset: {}", result.getRecordMetadata().offset());
        log.info("Send result topic: {}", result.getRecordMetadata().topic());
    }
}
