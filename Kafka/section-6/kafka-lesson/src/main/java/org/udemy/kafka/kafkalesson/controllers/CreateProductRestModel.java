package org.udemy.kafka.kafkalesson.controllers;

import java.math.BigDecimal;

public record CreateProductRestModel(String title, BigDecimal price, int quantity) {
}
