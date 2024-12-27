import {Routes} from "@angular/router";
import {NoTaskComponent} from "./tasks/no-task/no-task.component";
import {resolveUserName, UserTasksComponent} from "./users/user-tasks/user-tasks.component";
import {NotFoundComponent} from "./not-found/not-found.component";
import {routes as usersRoutes} from "./users/users.routes"

export const routes: Routes = [
  {path: '', component: NoTaskComponent},
  {
    path: 'users/:userId', component: UserTasksComponent,
    children: usersRoutes,
    data:{
      message:'Hello from data!!!'
    },
    resolve:{
      userName: resolveUserName,
    }
  },
  {path: '**', component: NotFoundComponent}
]
