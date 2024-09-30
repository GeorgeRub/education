package com.mvc.aopbefore.dao;

import com.mvc.aopbefore.models.Account;
import org.springframework.stereotype.Repository;

@Repository
public class AccountDAOImpl implements AccountDAO {
    @Override
    public void addAccount(Account account) {
        System.out.println(getClass().getName() + " --->>> Doing some work!!!");
        System.out.println("Account: " + account);
    }
}
