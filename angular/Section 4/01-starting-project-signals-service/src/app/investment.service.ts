import {Injectable, signal} from "@angular/core";
import type {InvestmentInput} from "./investment-input.model";
import type {InvestmentResult} from "./investment-result.model";

@Injectable({providedIn: 'root'})
export class InvestmentService {

  //resultData:InvestmentResult [] = []
  resultData = signal<InvestmentResult [] | undefined>(undefined)

  onCalculateInvestmentResults(data: InvestmentInput) {
    let {
      initialInvestment,
      annualInvestment,
      expectedReturn,
      duration
    } = data;
    const annualData = [];
    let investmentValue = initialInvestment;

    for (let i = 0; i < duration; i++) {
      const year = i + 1;
      const interestEarnedInYear = investmentValue * (expectedReturn / 100);
      investmentValue += interestEarnedInYear + annualInvestment;
      const totalInterest =
        investmentValue - annualInvestment * year - initialInvestment;
      annualData.push({
        year: year,
        interest: interestEarnedInYear,
        valueEndOfYear: investmentValue,
        annualInvestment: annualInvestment,
        totalInterest: totalInterest,
        totalAmountInvested: initialInvestment + annualInvestment * year,
      });
    }
    // console.log(annualData)
    // this.resultData.set(annualData);
    this.resultData.set(annualData)
  }

}
