import {Component, EventEmitter, Input, Output} from '@angular/core';
import {FormsModule} from "@angular/forms";
import {type NewTaskData} from "./new-task.model";
import {TasksService} from "../tasks.service";
import {inject} from "@angular/core";

@Component({
  selector: 'app-new-task',
  standalone: true,
  imports: [
    FormsModule
  ],
  templateUrl: './new-task.component.html',
  styleUrl: './new-task.component.css'
})
export class NewTaskComponent {

  @Input({required: true}) userId!: string;
  @Output() close = new EventEmitter<void>();
  // @Output() add = new EventEmitter<NewTaskData>();

  enteredTitle = ''
  enteredSummary = ''
  enteredDate = ''

  private tasksService = inject(TasksService)


  onCancel() {
    this.close.emit();
  }

  onSubmit() {
    this.tasksService.addTasks({
      title: this.enteredTitle,
      summary: this.enteredSummary,
      data: this.enteredDate
    }, this.userId)
    this.close.emit();
  }

}
