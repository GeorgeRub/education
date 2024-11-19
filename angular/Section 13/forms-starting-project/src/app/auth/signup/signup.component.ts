import {Component, DestroyRef, inject, OnInit} from '@angular/core';
import {
  AbstractControl,
  FormArray,
  FormControl,
  FormGroup,
  FormsModule,
  ReactiveFormsModule,
  Validators
} from "@angular/forms";
import {debounceTime} from "rxjs";

// function equalPasswords(control: AbstractControl) {
//   const password = control.get('password')?.value
//   const confirmPassword = control.get('confirmPassword')?.value
//   if (password === confirmPassword) {
//     return null;
//   }
//   return {passwordsNotEqual: true}
// }
function equalPasswords(controlVal1: string, controlVal2: string) {

  return (control: AbstractControl) => {
    const val1 = control.get(controlVal1)?.value
    const val2 = control.get(controlVal2)?.value
    if (val1 === val2) {
      return null;
    }
    return {valuesNotEqual: true}
  }


}

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
    }, {
      validators: [equalPasswords('password', 'confirmPassword')]
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
    if (this.welcomeForm.invalid) {
      console.log('INVALID FORM')
      return
    }
    console.log(this.welcomeForm.value)
  }

  onReset() {
    this.welcomeForm.reset()
  }

}
