package com.mvc.aopbefore;

import com.mvc.aopbefore.dao.AccountDAO;
import com.mvc.aopbefore.models.Account;
import org.springframework.boot.CommandLineRunner;
import org.springframework.boot.SpringApplication;
import org.springframework.boot.autoconfigure.SpringBootApplication;
import org.springframework.context.annotation.Bean;

@SpringBootApplication
public class AopBeforeApplication {

    public static void main(String[] args) {
        SpringApplication.run(AopBeforeApplication.class, args);
    }

    @Bean
    public CommandLineRunner commandLineRunner(AccountDAO accountDAO) {
        return runner -> {
            System.out.println("Hello");
            demoTheBeforeAdvice(accountDAO);
        };
    }

    private void demoTheBeforeAdvice(AccountDAO accountDAO) {
        Account account = new Account();
        account.setFirstName("John");
        account.setLastName("Doe");
        accountDAO.addAccount(account);
        System.out.println("Account after added " + account);
    }

}
