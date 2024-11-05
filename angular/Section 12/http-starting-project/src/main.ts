import {bootstrapApplication} from '@angular/platform-browser';

import {AppComponent} from './app/app.component';
import {HttpEventType, HttpHandlerFn, HttpRequest, provideHttpClient, withInterceptors} from "@angular/common/http";
import {tap} from "rxjs";

function loggingInterceptor(request: HttpRequest<unknown>, next: HttpHandlerFn) {
  // const req = request.clone({
  //   headers:request.headers.set('X-DEBUG','TESTING')
  // });
  console.log('Interceptor')
  console.log(request);
  return next(request).pipe(
    tap({
      next: event => {
        switch (event.type) {
          case HttpEventType.Response :{
            console.log(">>>--- TYPE Response ---<<<")
            console.log(event);
            console.log(event.status);
            console.log(event.body);
            break
          }
          case HttpEventType.ResponseHeader: {
            console.log(">>>--- TYPE ResponseHeader ---<<<")
            console.log(event);
            console.log(event.status);
            console.log(event);
          }
        }
      }
    })
  );
}

bootstrapApplication(AppComponent, {
  providers: [provideHttpClient(
    withInterceptors([loggingInterceptor])
  )]
}).catch((err) => console.error(err));
