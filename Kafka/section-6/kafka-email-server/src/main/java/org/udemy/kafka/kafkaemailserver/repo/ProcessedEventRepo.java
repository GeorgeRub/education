package org.udemy.kafka.kafkaemailserver.repo;

import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;
import org.udemy.kafka.kafkaemailserver.models.ProcessedEvent;

@Repository
public interface ProcessedEventRepo extends JpaRepository<ProcessedEvent, Long> {

    ProcessedEvent findByMessageId(String messageId);

}
