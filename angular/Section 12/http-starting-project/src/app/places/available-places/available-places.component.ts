import {Component, DestroyRef, inject, OnInit, signal} from '@angular/core';

import {Place} from '../place.model';
import {PlacesComponent} from '../places.component';
import {PlacesContainerComponent} from '../places-container/places-container.component';
import {PlacesService} from "../places.service";

@Component({
  selector: 'app-available-places',
  standalone: true,
  templateUrl: './available-places.component.html',
  styleUrl: './available-places.component.css',
  imports: [PlacesComponent, PlacesContainerComponent]
})
export class AvailablePlacesComponent implements OnInit {
  places = signal<Place[] | undefined>(undefined);
  isFetching = signal(false)
  error = signal('')
  private placesService = inject(PlacesService)
  private destroyRef = inject(DestroyRef)

  ngOnInit() {
    this.isFetching.set(true)
    const request = this.placesService
      .loadAvailablePlaces()
      .subscribe(
        {
          next: (places) => {
            console.log(places);
            this.places.set(places)
          },
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


  onSelectPlace(selectedPlace: Place) {
    const request = this.placesService.addPlaceToUserPlaces(selectedPlace)
      .subscribe({
        next: (place) => {
          console.log(place);
        }
      })

    this.destroyRef.onDestroy(() => {
      request.unsubscribe();
    })
  }

}
