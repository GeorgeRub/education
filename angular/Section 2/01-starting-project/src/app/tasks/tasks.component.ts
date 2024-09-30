import {Component, Input} from '@angular/core';
import {TasksService} from "./tasks.service";

@Component({
  selector: 'app-tasks',
  templateUrl: './tasks.component.html',
  styleUrl: './tasks.component.css'
})
export class TasksComponent {

  @Input({required: true}) selectedUser!: any;
  isAddingTask = false;

  constructor(private tasksService: TasksService) {
  }

  get listOfTasks() {
    return this.tasksService.getUserTasks(this.selectedUser.id)
  }

  onStartAddTask() {
    this.isAddingTask = true;
  }

  onCloseAddTask() {
    this.isAddingTask = false;
  }

}
