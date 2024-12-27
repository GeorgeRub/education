package com.mvc.manytomany.dao;

import com.mvc.manytomany.enity.Course;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface CourseDao extends JpaRepository<Course, Integer> {
    @Query("select c from Course c join fetch c.reviews where c.id = :theId")
    Course findCourseAndReviewsByCourseId(@Param("theId") int theId);

    void deleteCourseById(int theId);

    @Override
    Optional<Course> findById(Integer integer);

    //    Course findCourseById(int theId);
}
