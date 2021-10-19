import { HashRouter, Route, Switch } from 'react-router-dom';
import { Providers } from './providers';
import { Liquity } from './views/liquity';

export function Routes() {
  return (
    <>
      <HashRouter basename={'/'}>
        <Providers>
          <Switch> 
            <Route exact path="/liquity" component={() => <Liquity />} />
            <Route path="/" component={() => <div />} />
          </Switch>
        </Providers>
      </HashRouter>
    </>
  );
}
