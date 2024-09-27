import {Component, Input} from '@angular/core';
import {TaskComponent} from "./task/task.component";
import {NewTaskComponent} from "./new-task/new-task.component";
import {type NewTaskData} from "./new-task/new-task.model";
// import {from} from "rxjs";
import {TasksService} from "./tasks.service";

@Component({
  selector: 'app-tasks',
  standalone: true,
  imports: [
    TaskComponent,
    NewTaskComponent
  ],
  templateUrl: './tasks.component.html',
  styleUrl: './tasks.component.css'
})
export class TasksComponent {

  @Input({required: true}) selectedUser!: any;
  isAddingTask = false;

  // private tasksService = new TasksService();

  constructor(private tasksService: TasksService) {
  }

  get listOfTasks() {
    return this.tasksService.getUserTasks(this.selectedUser.id)
  }

  onCompleteTask(id: string) {
    return this.tasksService.removeTask(id)
  }

  onStartAddTask() {
    this.isAddingTask = true;
  }

  onCloseAddTask() {
    this.isAddingTask = false;
  }

  // onAddTask(taskData: NewTaskData) {
  //   this.tasksService.addTasks(taskData, this.selectedUser.id)
  //   this.isAddingTask = false
  // }

}
