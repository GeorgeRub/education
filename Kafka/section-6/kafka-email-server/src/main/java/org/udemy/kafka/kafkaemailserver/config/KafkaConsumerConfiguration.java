package org.udemy.kafka.kafkaemailserver.config;

import lombok.AllArgsConstructor;
import org.apache.kafka.clients.consumer.ConsumerConfig;
import org.apache.kafka.clients.producer.ProducerConfig;
import org.apache.kafka.common.serialization.StringDeserializer;
import org.apache.kafka.common.serialization.StringSerializer;
import org.springframework.context.annotation.Bean;
import org.springframework.context.annotation.Configuration;
import org.springframework.core.env.Environment;
import org.springframework.kafka.config.ConcurrentKafkaListenerContainerFactory;
import org.springframework.kafka.core.*;
import org.springframework.kafka.listener.DeadLetterPublishingRecoverer;
import org.springframework.kafka.listener.DefaultErrorHandler;
import org.springframework.kafka.support.serializer.ErrorHandlingDeserializer;
import org.springframework.kafka.support.serializer.JsonDeserializer;
import org.springframework.kafka.support.serializer.JsonSerializer;
import org.springframework.util.backoff.FixedBackOff;
import org.udemy.kafka.kafkaemailserver.exceptions.NotRetryableException;
import org.udemy.kafka.kafkaemailserver.exceptions.RetryableException;

import java.util.HashMap;
import java.util.Map;

/**
 * Configuration class for Kafka consumer.
 *
 * <p>It creates:</p>
 * <ul>
 *     <li>{@link ConsumerFactory}</li>
 *     <li>{@link ConcurrentKafkaListenerContainerFactory}</li>
 *     <li>{@link KafkaTemplate}</li>
 * </ul>
 *
 * @author Roman Pronin
 * @since 1.0
 */
@Configuration
@AllArgsConstructor
public class KafkaConsumerConfiguration {

    Environment environment;

    /**
     * Creates a {@link ConsumerFactory} which is used to create {@link org.apache.kafka.clients.consumer.KafkaConsumer}s
     * for each listener in the application.
     *
     * @return a {@link ConsumerFactory} instance.
     */
    @Bean
    ConsumerFactory<String, Object> consumerFactory() {
        Map<String, Object> config = new HashMap<>();

        config.put(ConsumerConfig.BOOTSTRAP_SERVERS_CONFIG, environment.getProperty("spring.kafka.consumer.bootstrap-servers"));
        config.put(ConsumerConfig.KEY_DESERIALIZER_CLASS_CONFIG, StringDeserializer.class);
        config.put(JsonDeserializer.TRUSTED_PACKAGES, environment.getProperty("spring.kafka.properties.spring.json.trusted.packages"));
        config.put(ConsumerConfig.GROUP_ID_CONFIG, environment.getProperty("spring.kafka.consumer.group-id"));

        // Error handling
        config.put(ConsumerConfig.VALUE_DESERIALIZER_CLASS_CONFIG, ErrorHandlingDeserializer.class);
        config.put(ErrorHandlingDeserializer.VALUE_DESERIALIZER_CLASS, JsonDeserializer.class);

        return new DefaultKafkaConsumerFactory<>(config);
    }

    /**
     * Creates a {@link ConcurrentKafkaListenerContainerFactory} which is used to create containers for each listener
     * in the application.
     *
     * @param consumerFactory a {@link ConsumerFactory} to be used to create {@link org.apache.kafka.clients.consumer.KafkaConsumer}s
     * @param kafkaTemplate   a {@link KafkaTemplate} that is used by the {@link DeadLetterPublishingRecoverer} to send
     *                        error messages to another topic.
     * @return a {@link ConcurrentKafkaListenerContainerFactory} instance.
     */
    @Bean
    ConcurrentKafkaListenerContainerFactory<String, Object> kafkaListenerContainerFactory(
            ConsumerFactory<String, Object> consumerFactory, KafkaTemplate<String, Object> kafkaTemplate
    ) {

        ConcurrentKafkaListenerContainerFactory<String, Object> factory = new ConcurrentKafkaListenerContainerFactory<>();
        factory.setConsumerFactory(consumerFactory());

        // Error handling
        // use this for move error messages to another topic
        DefaultErrorHandler errorHandler = new DefaultErrorHandler(
                new DeadLetterPublishingRecoverer(kafkaTemplate),
                new FixedBackOff(5000, 3));
        errorHandler.addNotRetryableExceptions(NotRetryableException.class);
        errorHandler.addRetryableExceptions(RetryableException.class);
        factory.setCommonErrorHandler(errorHandler);
        return factory;
    }


    /**
     * Creates a {@link KafkaTemplate} which is used to send messages to Kafka topics.
     *
     * @param producerFactory a {@link ProducerFactory} to be used to create {@link org.apache.kafka.clients.producer.KafkaProducer}s
     * @return a {@link KafkaTemplate} instance.
     */
    @Bean
    KafkaTemplate<String, Object> kafkaTemplate(ProducerFactory<String, Object> producerFactory) {
        return new KafkaTemplate<>(producerFactory);
    }

    /**
     * Creates a {@link ProducerFactory} which is used to create {@link org.apache.kafka.clients.producer.KafkaProducer}s
     * for sending messages to Kafka topics.
     *
     * @return a {@link ProducerFactory} instance.
     */
    @Bean
    ProducerFactory<String, Object> producerFactory() {
        Map<String, Object> config = new HashMap<>();
        config.put(ProducerConfig.BOOTSTRAP_SERVERS_CONFIG, environment.getProperty("spring.kafka.consumer.bootstrap-servers"));
        config.put(ProducerConfig.VALUE_SERIALIZER_CLASS_CONFIG, JsonSerializer.class);
        config.put(ProducerConfig.KEY_SERIALIZER_CLASS_CONFIG, StringSerializer.class);
        return new DefaultKafkaProducerFactory<>(config);
    }

}
