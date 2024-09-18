package com.mvc.validation.model;

import com.mvc.validation.validation.CourseCode;
import jakarta.validation.constraints.*;
import lombok.Data;

@Data
public class Customer {

    private String firstName;

    @NotNull(message = "Should not be empty")
    @Size(min=1, message = "Should be more than 1 character")
    private String lastName;

    @NotNull(message = "Should not be empty")
    @Min(value=0, message="must be greater than or equal to zero")
    @Max(value=10, message="must be less than or equal to 10")
    private Integer freePasses;

    @NotNull(message = "Should not be empty")
    @Pattern(regexp = "^[a-zA-z0-9]{5}", message = "Only 5 characters/digits")
    private String postalCode;

    @CourseCode
    private String courseCode;

}
