package com.mvc.validation.model;

import jakarta.validation.constraints.NotNull;
import jakarta.validation.constraints.Size;
import lombok.Data;

@Data
public class Customer {

    private String firstName;

    @NotNull(message = "Should not be empty")
    @Size(min=1, message = "Should be more than 1 character")
    private String lastName;

}
