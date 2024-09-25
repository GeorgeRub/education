package org.securitydemo.onetomany.dao;

import org.securitydemo.onetomany.entity.InstructorDetail;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface InstructorDetailDao extends JpaRepository<InstructorDetail, Integer> {
}
