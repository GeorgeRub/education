package org.securitydemo.onetoonetest.dao;

import org.securitydemo.onetoonetest.entity.InstructorDetail;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface InstructorDetailDao extends JpaRepository<InstructorDetail, Integer> {
}
