package com.mvc.manytomany.dao;

import com.mvc.manytomany.enity.Instructor;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface InstructorDao extends JpaRepository<Instructor, Integer> {
}
