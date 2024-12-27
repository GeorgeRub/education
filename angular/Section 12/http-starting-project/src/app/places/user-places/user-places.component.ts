import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';

import {PlacesContainerComponent} from '../places-container/places-container.component';
import {PlacesComponent} from '../places.component';
import {Place} from "../place.model";
import {PlacesService} from "../places.service";

@Component({
  selector: 'app-user-places',
  standalone: true,
  templateUrl: './user-places.component.html',
  styleUrl: './user-places.component.css',
  imports: [PlacesContainerComponent, PlacesComponent],
})
export class UserPlacesComponent implements OnInit {
  // places = signal<Place[] | undefined>(undefined);
  isFetching = signal(false)
  error = signal('')
  private placesService = inject(PlacesService)
  private destroyRef = inject(DestroyRef)
  places = this.placesService.loadedUserPlaces

  ngOnInit() {
    this.isFetching.set(true)
    const request = this.placesService
      .loadUserPlaces()
      .subscribe(
        {
          complete: () => {
            this.isFetching.set(false)
          },
          error: (error: Error) => {
            this.isFetching.set(false);
            this.error.set(error.message);
          }
        }
      );

    this.destroyRef.onDestroy(() => {
      request.unsubscribe();
    })

  }

  onRemovePlace(event: Place) {
    const request = this.placesService.removeUserPlace(event).subscribe();
    this.destroyRef.onDestroy(() => {
      request.unsubscribe();
    })
  }
}
