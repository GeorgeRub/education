import {Component, inject, input} from '@angular/core';
import {UsersService} from "../users.service";
import {ActivatedRouteSnapshot, ResolveFn, RouterLink, RouterOutlet, RouterStateSnapshot} from "@angular/router";

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
export class UserTasksComponent {
  message = input.required<string>()
  userName = input.required<string>()
}

export const resolveUserName: ResolveFn<string> = (
  activatedRoute: ActivatedRouteSnapshot,
  routerSate: RouterStateSnapshot
) => {
  const userService = inject(UsersService)
  return userService.users.find(user => user.id === activatedRoute.paramMap.get('userId'))?.name || ''
}
