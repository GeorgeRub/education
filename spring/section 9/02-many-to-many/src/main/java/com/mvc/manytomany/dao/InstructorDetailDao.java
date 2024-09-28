package com.mvc.manytomany.dao;

import com.mvc.manytomany.enity.InstructorDetail;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface InstructorDetailDao extends JpaRepository<InstructorDetail, Integer> {
}
