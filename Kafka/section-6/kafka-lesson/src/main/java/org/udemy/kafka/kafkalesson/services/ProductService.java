package org.udemy.kafka.kafkalesson.services;

import org.springframework.kafka.support.SendResult;
import org.udemy.kafka.kafkacore.models.ProductCreatedEvent;
import org.udemy.kafka.kafkalesson.controllers.CreateProductRestModel;

import java.util.concurrent.CompletableFuture;

public interface ProductService {

    String createProduct(CreateProductRestModel productRestModel) throws Exception;
    CompletableFuture<SendResult<String, ProductCreatedEvent>> createProductAsync(CreateProductRestModel productRestModel);

}
