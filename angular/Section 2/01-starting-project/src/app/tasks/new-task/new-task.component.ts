import {Component, EventEmitter, inject, Input, Output} from '@angular/core';
import {TasksService} from "../tasks.service";

@Component({
  selector: 'app-new-task',
  templateUrl: './new-task.component.html',
  styleUrl: './new-task.component.css'
})
export class NewTaskComponent {

  @Input({required: true}) userId!: string;
  @Output() close = new EventEmitter<void>();

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
