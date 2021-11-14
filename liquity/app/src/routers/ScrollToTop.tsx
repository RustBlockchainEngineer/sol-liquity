import { useEffect } from "react";
import { RouterProps, withRouter } from "react-router-dom";

export interface ScrollToTopProps {
  history: RouterProps["history"];
}

const ScrollToTop: React.FC<ScrollToTopProps> = ({ history }) => {
  useEffect(() => {
    const unlisten = history.listen(() => {
      window.scrollTo(0, 0);
    });
    return () => {
      unlisten();
    };
  }, [history]);

  return null;
};

export default withRouter(ScrollToTop);
