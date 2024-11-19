import {Component, inject, input, OnInit, signal} from '@angular/core';
import {FormsModule} from '@angular/forms';

import {TasksService} from '../tasks.service';
import {Router, RouterLink} from "@angular/router";

@Component({
  selector: 'app-new-task',
  standalone: true,
  imports: [FormsModule, RouterLink],
  templateUrl: './new-task.component.html',
  styleUrl: './new-task.component.css',
})
export class NewTaskComponent implements OnInit {
  userId = input.required<string>();
  enteredTitle = signal('');
  enteredSummary = signal('');
  enteredDate = signal('');
  private tasksService = inject(TasksService);
  private router = inject(Router)

  ngOnInit() {
    console.log('new tasks user id is', this.userId())
  }

  onSubmit() {
    this.tasksService.addTask(
      {
        title: this.enteredTitle(),
        summary: this.enteredSummary(),
        date: this.enteredDate(),
      },
      this.userId()
    );
    this.router.navigate(['/users', this.userId, 'tasks'], {replaceUrl: true})
  }
}
