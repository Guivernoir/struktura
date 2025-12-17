import { useOutletContext } from 'react-router-dom';
import Hero from '../components/Hero';
import Ticker from '../components/Ticker';
import ModeCards from '../components/ModeCards';

const Home = () => {
  const { t } = useOutletContext();

  return (
    <>
      <Hero t={t} />
      <Ticker t={t} />
      <ModeCards t={t} />
    </>
  );
};

export default Home;