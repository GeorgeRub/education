package org.securitydemo.onetoonetest.dao;

import org.securitydemo.onetoonetest.entity.Instructor;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface InstructorDao extends JpaRepository<Instructor, Integer> {
}
