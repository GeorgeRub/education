package org.securitydemo.onetomany.dao;

import org.securitydemo.onetomany.entity.Instructor;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;
import org.springframework.stereotype.Repository;

import java.util.Optional;

@Repository
public interface InstructorDao extends JpaRepository<Instructor, Integer> {

    @Query("select i from Instructor i JOIN FETCH i.courses join fetch i.instructorDetail where i.id = :courseId")
    public Instructor findByCoursesId(@Param("courseId") int coursesId);

    @Query("select i from Instructor i JOIN FETCH i.courses where i.id = :id")
    public Optional<Instructor> findById(@Param("id") int id);

}
