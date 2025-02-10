package org.udemy.kafka.kafkalesson.controllers;

import lombok.AllArgsConstructor;
import lombok.extern.slf4j.Slf4j;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.kafka.support.SendResult;
import org.springframework.web.bind.annotation.PostMapping;
import org.springframework.web.bind.annotation.RequestBody;
import org.springframework.web.bind.annotation.RequestMapping;
import org.springframework.web.bind.annotation.RestController;
import org.udemy.kafka.kafkacore.models.ProductCreatedEvent;
import org.udemy.kafka.kafkalesson.services.ProductService;

import java.util.Date;
import java.util.concurrent.CompletableFuture;
import java.util.concurrent.ExecutionException;

@RestController
@RequestMapping("/products")
@AllArgsConstructor
@Slf4j
public class ProductController {

    private final ProductService productService;

    @PostMapping
    @RequestMapping("/async")
    public ResponseEntity<Object> createProductAsync(@RequestBody CreateProductRestModel productRestModel) throws ExecutionException, InterruptedException {
        CompletableFuture<SendResult<String, ProductCreatedEvent>> productId = null;
        try {
            productId = productService.createProductAsync(productRestModel);
        } catch (Exception e) {
            log.error("Error sending message {}", e.getMessage());
            ErrorMessage errorMessage = ErrorMessage
                    .builder()
                    .timestamp(new Date())
                    .message(e.getMessage())
                    .details("/products/async")
                    .build();
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(errorMessage);
        }
        return ResponseEntity.status(HttpStatus.CREATED).body(productId.get().getProducerRecord().key());
    }

    @PostMapping
    public ResponseEntity<Object> createProduct(@RequestBody CreateProductRestModel productRestModel) {
        try {
            String productId = productService.createProduct(productRestModel);
            return ResponseEntity.status(HttpStatus.CREATED).body(productId);
        } catch (Exception e) {
            log.error("Error sending message {}", e.getMessage());
            ErrorMessage errorMessage = ErrorMessage
                    .builder()
                    .timestamp(new Date())
                    .message(e.getMessage())
                    .details("/products")
                    .build();
            return ResponseEntity.status(HttpStatus.INTERNAL_SERVER_ERROR).body(errorMessage);
        }

    }

}
