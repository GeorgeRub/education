package org.securitydemo.onetomany.dao;

import org.securitydemo.onetomany.entity.Review;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface ReviewDao extends JpaRepository<Review, Integer> {
}
