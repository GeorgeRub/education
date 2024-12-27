import {Component, DestroyRef, inject, OnInit} from '@angular/core';
import {AbstractControl, FormControl, FormGroup, FormsModule, ReactiveFormsModule, Validators} from "@angular/forms";
import {debounceTime, of} from "rxjs";

function mustContentQuestionMark(control: AbstractControl) {
  console.log('custom validating')
  if (control.value.includes('?')) {
    return null
  }
  return {doesNotContainQuestionMark: true}

}

function emailIsUnique(control: AbstractControl) {
  if (control.value !== 'test@example.com') {
    console.log('return null')
    return of(null)
  }
  console.log('return NOT null')
  return of({notUnique: true})
}

//This works only on NOT server side
let initialedEmail = '';
const savedForm = window.localStorage.getItem('saved-login-form')
if (savedForm) {
  const loadedForm = JSON.parse(savedForm)
  initialedEmail = loadedForm.email
}

@Component({
  selector: 'app-login',
  standalone: true,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
  imports: [
    FormsModule, ReactiveFormsModule
  ]
})
export class LoginComponent implements OnInit {

  destroyRef = inject(DestroyRef)

  inputForm = new FormGroup({
    email: new FormControl(initialedEmail, {
      validators: [Validators.required, Validators.email],
      asyncValidators: [emailIsUnique]
    }),
    password: new FormControl('', {
      validators: [Validators.required, Validators.minLength(6), mustContentQuestionMark]
    }),
  })

  get emailIsInvalid() {
    return this.inputForm.controls.email.invalid
      && this.inputForm.controls.email.touched
      && this.inputForm.controls.email.dirty;
  }

  get passwordIsInvalid() {
    return this.inputForm.controls.password.invalid
      && this.inputForm.controls.password.touched
      && this.inputForm.controls.password.dirty;
  }

  ngOnInit() {

    //This will work on a server side
    //
    // const savedForm = window.localStorage.getItem('saved-login-form')
    // if (savedForm) {
    //   const loadedForm = JSON.parse(savedForm)
    //   // this.inputForm.controls.email.setValue(loadedForm.email)
    //   this.inputForm.patchValue({
    //     email: loadedForm.email
    //   })
    // }


    const valChange = this.inputForm.valueChanges
      .pipe(debounceTime(800))
      .subscribe({
        next: value => {
          window.localStorage.setItem('saved-login-form', JSON.stringify({email: value.email}))
        }
      })
    this.destroyRef.onDestroy(() => {
      valChange.unsubscribe()
    })
  }

  onSubmit() {
    console.log(this.inputForm)
  }

}
