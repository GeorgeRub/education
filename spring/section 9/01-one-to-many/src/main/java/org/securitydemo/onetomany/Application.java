package org.securitydemo.onetomany;

import lombok.extern.slf4j.Slf4j;
import org.hibernate.Hibernate;
import org.securitydemo.onetomany.dao.CourseDao;
import org.securitydemo.onetomany.dao.InstructorDao;
import org.securitydemo.onetomany.dao.InstructorDetailDao;
import org.securitydemo.onetomany.entity.Course;
import org.securitydemo.onetomany.entity.Instructor;
import org.securitydemo.onetomany.entity.InstructorDetail;
import org.securitydemo.onetomany.entity.Review;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;

import java.util.List;
import java.util.Optional;

@SpringBootApplication
@Slf4j
public class Application {

    private CourseDao courseDao;
    private InstructorDao instructorDao;
    private InstructorDetailDao instructorDetailDao;

    public static void main(String[] args) {
        SpringApplication.run(Application.class, args);
    }

    @Bean
    public CommandLineRunner commandLineRunner(InstructorDetailDao instructorDetailDao, InstructorDao instructorDao, CourseDao courseDao) {
        this.instructorDao = instructorDao;
        this.instructorDetailDao = instructorDetailDao;
        this.courseDao = courseDao;
        return args -> {
//            createInstructorWithCourse();
//            findInstructorByCourseId(1);
            updateInstructor(1);

        };
    }

    private void updateInstructor(int i) {
        Optional<Instructor> instructorOptional = instructorDao.findById(i);
        if (instructorOptional.isPresent()) {
            Review review1 = Review.builder().comment("Some review 1").build();
            Review review2 = Review.builder().comment("Some review 2").build();
            Review review3 = Review.builder().comment("Some review 3").build();
            Review review4 = Review.builder().comment("Some review 4").build();
            Instructor instructor = instructorOptional.get();
            List<Course> course = instructor.getCourses();
            System.out.println("---- course ----");
            System.out.println(course);
            instructor.getCourses().get(0).getReviews().add(review1);
            instructor.getCourses().get(0).getReviews().add(review2);
            instructor.getCourses().get(1).getReviews().add(review3);
            instructor.getCourses().get(1).getReviews().add(review4);
            courseDao.saveAll(course);
        }

    }

    private void findInstructorByCourseId(int i) {
        System.out.println("--------------------------");
        Instructor instructor = instructorDao.findByCoursesId(i);
        System.out.println(instructor);
        System.out.println("<<<>>>");
        System.out.println(instructor.getCourses());
        System.out.println("---------------------------");
    }

    private void createInstructorWithCourse() {
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

        Course course1 = Course.builder().title("Air Guitar").build();
        Course course2 = Course.builder().title("Ping-pong").build();
        instructor.add(course1);
        instructor.add(course2);
        instructorDao.save(instructor);
        log.info("Done!!!");
    }

}
