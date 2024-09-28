package com.mvc.manytomany.dao;

import com.mvc.manytomany.enity.Review;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.stereotype.Repository;

@Repository
public interface ReviewDao extends JpaRepository<Review, Integer> {
}
