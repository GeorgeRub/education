import {inject, Injectable, signal} from '@angular/core';

import {Place} from './place.model';
import {catchError, map, tap, throwError} from "rxjs";
import {HttpClient} from "@angular/common/http";
import {ErrorService} from "../shared/error.service";

@Injectable({
  providedIn: 'root',
})
export class PlacesService {
  private userPlaces = signal<Place[]>([]);
  private httpClient = inject(HttpClient)
  private errorService = inject(ErrorService)

  loadedUserPlaces = this.userPlaces.asReadonly();

  loadAvailablePlaces() {
    return this.fetchPlaces(
      "http://localhost:3000/places",
      "Something went wrong fetching favorite places. Please try again later."
    );
  }

  loadUserPlaces() {
    return this.fetchPlaces(
      "http://localhost:3000/user-places",
      "Something went wrong fetching your favorite places. Please try again later."
    ).pipe(tap({
      next: (place) => {
        console.log(place);
        this.userPlaces.set(place)
      }
    }));
  }

  addPlaceToUserPlaces(place: Place) {
    const prevPlaces = this.userPlaces()
    if (!prevPlaces.some((p) => p.id === place.id)) {
      this.userPlaces.set([...prevPlaces, place])
    }
    return this.httpClient.put('http://localhost:3000/user-places', {
      placeId: place.id
    }).pipe(
      catchError(error => {
        this.userPlaces.set(prevPlaces)
        this.errorService.showError('Failed to store selected place.')
        return throwError(() => new Error(error.message));
      }),
    );
  }

  removeUserPlace(place: Place) {
    const prevPlaces = this.userPlaces()
    if (prevPlaces.some((p) => p.id === place.id)) {
      this.userPlaces.set(prevPlaces.filter(p => p.id !== place.id))
    }
    return this.httpClient.delete('http://localhost:3000/user-places/'+place.id)
      .pipe(catchError(error => {
        this.userPlaces.set(prevPlaces)
        this.errorService.showError('Failed to remove the selected place.')
        return throwError(() => new Error('Failed to remove the selected place.'));
      }))

  }

  private fetchPlaces(url: string, errorMessage: string) {
    return this.httpClient
      .get<{ places: Place[] }>(url)
      .pipe(
        map((responseData) => responseData.places),
        catchError((error) => throwError(() => new Error(errorMessage)))
      )
  }
}