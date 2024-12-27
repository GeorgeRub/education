import {NgModule} from "@angular/core";
import {TasksComponent} from "./tasks.component";
import {TaskComponent} from "./task/task.component";
import {NewTaskComponent} from "./new-task/new-task.component";
import {BrowserModule} from "@angular/platform-browser";
import {DatePipe} from "@angular/common";
import {FormsModule} from "@angular/forms";
import {SharedModule} from "../shared/shared.module";

@NgModule({
  declarations: [TasksComponent, TaskComponent, NewTaskComponent],
  exports:[TasksComponent],
  imports: [BrowserModule, DatePipe, FormsModule, SharedModule]
})
export class TasksModule { }
