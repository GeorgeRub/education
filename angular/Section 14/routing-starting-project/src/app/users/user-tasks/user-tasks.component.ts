import {Component, computed, DestroyRef, inject, input, OnInit} from '@angular/core';
import {UsersService} from "../users.service";
import {ActivatedRoute, RouterLink, RouterOutlet} from "@angular/router";

@Component({
  selector: 'app-user-tasks',
  standalone: true,
  templateUrl: './user-tasks.component.html',
  styleUrl: './user-tasks.component.css',
  imports: [
    RouterOutlet,
    RouterLink
  ]
})
export class UserTasksComponent implements OnInit {
  userId = input.required<string>()
  private usersService = inject(UsersService)

  private activatedRoute = inject(ActivatedRoute)
  private destroyRef = inject(DestroyRef)
  userName = ''
  // userName = computed(() =>
  //   this.usersService.users
  //     .find(u => u.id === this.userId())?.name)


  ngOnInit() {
    console.log(this.activatedRoute)
    const subscribed = this.activatedRoute.paramMap.subscribe({
      next: paramMap => {
        this.userName = this.usersService.users.find(user => user.id === paramMap.get('userId'))?.name || ''
      }
    })

    this.destroyRef.onDestroy(() => {
      subscribed.unsubscribe()
    })

  }

}
