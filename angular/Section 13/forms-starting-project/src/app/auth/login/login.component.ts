import {afterNextRender, Component, DestroyRef, inject, viewChild} from '@angular/core';
import {FormsModule, NgForm} from "@angular/forms";
import {debounceTime} from "rxjs";

@Component({
  selector: 'app-login',
  standalone: true,
  templateUrl: './login.component.html',
  styleUrl: './login.component.css',
  imports: [
    FormsModule
  ]
})
export class LoginComponent {
  private form = viewChild.required<NgForm>('form')
  private destryRef = inject(DestroyRef)

  constructor() {
    afterNextRender(() => {

      const savedForm = window.localStorage.getItem('saved-login-form')
      if (savedForm) {
        const loadedForm = JSON.parse(savedForm)
        const savedEmail = loadedForm.email
        setTimeout(
          ()=> {
            this.form().controls['email'].setValue(savedEmail)
          },1
        )

        // this.form().setValue({
        //   email: savedEmail,
        //   password: ''
        // })
      }

      const subscription = this.form().valueChanges?.pipe(
        debounceTime(800)
      ).subscribe({
        next: (value) => {
          console.log(value)
          window.localStorage.setItem(
            'saved-login-form',
            JSON.stringify({email: value.email, password: value.password}))
        }
      })
      this.destryRef.onDestroy(() => {
        subscription?.unsubscribe()
      })
    })
  }

//   isEmailValid(): boolean{
// form
//   }

  onSubmit(formData: NgForm) {
    if (formData.form.invalid) return
    const email = formData.form.value.email
    const password = formData.form.value.password
    console.log(email, 'and', password)
    formData.form.reset()
    // formData.form.disable()
  }
}
