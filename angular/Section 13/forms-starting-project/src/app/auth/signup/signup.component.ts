import {Component, DestroyRef, inject, OnInit} from '@angular/core';
import {FormArray, FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators} from "@angular/forms";
import {debounceTime} from "rxjs";

@Component({
  selector: 'app-signup',
  standalone: true,
  templateUrl: './signup.component.html',
  styleUrl: './signup.component.css',
  imports: [
    ReactiveFormsModule, FormsModule
  ]
})
export class SignupComponent implements OnInit {

  destroyRef = inject(DestroyRef)

  welcomeForm = new FormGroup({
    email: new FormControl('', {
      validators: [Validators.required, Validators.email]
    }),
    passwords: new FormGroup({
      password: new FormControl('', {
        validators: [Validators.required, Validators.minLength(6)]
      }),
      confirmPassword: new FormControl('', {
        validators: [Validators.required, Validators.minLength(6)]
      }),
    }),
    firstName: new FormControl('', {
      validators: [Validators.required]
    }),
    lastName: new FormControl('', {validators: [Validators.required]}),
    address: new FormGroup({
      street: new FormControl('', {validators: [Validators.required]}),
      number: new FormControl('', {validators: [Validators.required]}),
      postalCode: new FormControl('', {validators: [Validators.required]}),
      city: new FormControl('', {validators: [Validators.required]}),
    }),
    role: new FormControl<'student' | 'teacher' | 'employee' | 'founder' | 'other'>
    ('student', {validators: [Validators.required]}),
    agree: new FormControl(false, {validators: [Validators.required]}),
    source: new FormArray([
      new FormControl(false),
      new FormControl(false),
      new FormControl(false),
    ])
  })

  ngOnInit() {

    const savedWelcomeForm = window.localStorage.getItem('welcome-form')
    if (savedWelcomeForm) {
      const loadedWelcomeForm = JSON.parse(savedWelcomeForm)
      this.welcomeForm.patchValue({
        email: loadedWelcomeForm.email
      })
    }

    const valChanged = this.welcomeForm
      .valueChanges
      .pipe(debounceTime(800)).subscribe({
        next: value => {
          window.localStorage.setItem('welcome-form', JSON.stringify({email: value.email}))
        }
      })

    this.destroyRef.onDestroy(() => {
      valChanged.unsubscribe()
    })
  }

  onSubmit() {
    console.log(this.welcomeForm.value)
  }

  onReset() {
    this.welcomeForm.reset()
  }

}
