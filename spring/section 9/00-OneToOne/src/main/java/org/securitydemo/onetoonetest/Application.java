package org.securitydemo.onetoonetest;

import lombok.extern.slf4j.Slf4j;
import org.securitydemo.onetoonetest.dao.InstructorDao;
import org.securitydemo.onetoonetest.entity.Instructor;
import org.securitydemo.onetoonetest.entity.InstructorDetail;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;

import java.util.Optional;

@SpringBootApplication
@Slf4j
public class Application {

    InstructorDao instructorDao;

    public static void main(String[] args) {
        SpringApplication.run(Application.class, args);
    }

    @Bean
    public CommandLineRunner commandLineRunner(InstructorDao instructorDao) {
        this.instructorDao = instructorDao;
        return runner -> {
//            createInstructor(instructorDao);
            findInstructor();
        };

    }

    private void findInstructor() {
        Optional<Instructor> instructor = instructorDao.findById(1);
        if (instructor.isPresent()) {
            log.info("Instructor found {}", instructor.get());
        } else {
            log.info("Instructor not found");
        }
    }

    private void createInstructor() {
        InstructorDetail instructorDetail = InstructorDetail
                .builder()
                .hobby("Codding")
                .youtubeChannel("some channel")
                .build();

        Instructor instructor = Instructor
                .builder()
                .firstName("Chad")
                .lastName("Darby")
                .email("darby@gogle.com")
                .instructorDetail(instructorDetail)
                .build();

        log.info("sawing instructor {}", instructor);
        this.instructorDao.save(instructor);
        log.info("Done!!!");
    }

}
