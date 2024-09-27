import {Component, EventEmitter, Input, Output} from '@angular/core';
import {NgOptimizedImage} from "@angular/common";
import {type User} from "./user.model";
import {CardComponent} from "../shared/card/card.component";

@Component({
  selector: 'app-user',
  standalone: true,
  imports: [
    NgOptimizedImage,
    CardComponent
  ],
  templateUrl: './user.component.html',
  styleUrl: './user.component.css'
})
export class UserComponent {
  @Input({required: true}) user!: User
  @Input({required: true}) isSelectedUser!: boolean
  @Output() selected = new EventEmitter<string>();

  get imagePath() {
    return 'assets/users/' + this.user.avatar
  }

  onSelectUser() {
    this.selected.emit(this.user.id);
  }

}
