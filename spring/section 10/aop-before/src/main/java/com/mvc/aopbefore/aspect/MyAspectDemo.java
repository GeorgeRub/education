package com.mvc.aopbefore.aspect;

import com.mvc.aopbefore.models.Account;
import org.aspectj.lang.JoinPoint;
import org.aspectj.lang.annotation.Aspect;
import org.aspectj.lang.annotation.Before;
import org.springframework.stereotype.Component;

@Aspect
@Component
public class MyAspectDemo {

    @Before("execution(public void addAccount(..))")
    public void beforeAddAccount(JoinPoint joinPoint) {
        System.out.println("before add account");
        System.out.println(joinPoint.getSignature());
        Object[] args = joinPoint.getArgs();
        for (Object arg : args) {
            if (arg instanceof Account account) {
                System.out.println("Account aspect " + account);
                account.setLastName(account.getLastName().toUpperCase());

            }
        }
    }

}
