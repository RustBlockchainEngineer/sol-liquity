import { HashRouter, Route, Switch } from 'react-router-dom';
import { Providers } from './providers';
import { LiquityView } from './views/liquity';

export function Routes() {
  return (
    <>
      <HashRouter basename={'/'}>
        <Providers>
          <Switch> 
            <Route exact path="/liquity" component={() => <LiquityView />} />
            <Route path="/" component={() => <LiquityView />} />
          </Switch>
        </Providers>
      </HashRouter>
    </>
  );
}
