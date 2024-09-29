package com.mvc.aopbefore.dao;

import org.springframework.stereotype.Repository;

@Repository
public class AccountDAOImpl implements AccountDAO {
    @Override
    public void addAccount() {
        System.out.println(getClass().getName() + " --->>> Doing some work!!!");
    }
}
