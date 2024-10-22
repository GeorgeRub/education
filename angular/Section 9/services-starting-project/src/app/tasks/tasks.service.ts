import {Injectable, signal} from '@angular/core';
import {Task, TaskStatus} from "./task.model";

@Injectable({
  providedIn: 'root'
})
export class TasksService {

  tasks = signal<Task[]>([])

  addTask(taskData: { title: string, description: string }) {
    const newTasks: Task = {
      ...taskData,
      id: Math.random().toString(),
      status: 'OPEN'
    }
    this.tasks.update((oldTasks) => [...oldTasks, newTasks]);
  }
}
