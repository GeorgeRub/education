import {Component, OnInit} from '@angular/core';
import {DashboardItemComponent} from "../dashboard-item/dashboard-item.component";

@Component({
  selector: 'app-server-status',
  standalone: true,
  imports: [
    DashboardItemComponent
  ],
  templateUrl: './server-status.component.html',
  styleUrl: './server-status.component.css'
})
export class ServerStatusComponent implements OnInit {
  currentStatus: 'online' | 'offline' | 'unknown' = 'online';

  constructor() {
    console.log('Server Status Component loaded on constructor');
  }

  ngOnInit(){
    console.log('Server Status Component loaded on initialized.');
    setInterval(() => {
      const rnd = Math.random();
      if(rnd < 0.5){
        this.currentStatus = 'online';
      }else if (rnd < 0.9){
        this.currentStatus = 'offline';
      }else{
        this.currentStatus = 'unknown';
      }
    },5000)
  }

}
