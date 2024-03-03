import type { FC } from 'react';
import { useEffect } from 'react';

type SvgViewerProps = {
  svgString: string;
  err: string;
  score: number;
};

const SvgViewer: FC<SvgViewerProps> = ({ svgString, err, score }) => {
  useEffect(() => {
    // eslint-disable-next-line @typescript-eslint/no-unsafe-call, @typescript-eslint/no-explicit-any, @typescript-eslint/no-unsafe-member-access
    (window as any).refreshEvents();
  }, [svgString]);

  return (
    <>
      <div>
        score={score} {err && <span style={{ color: 'red' }}>({err})</span>}
      </div>
      <div
        dangerouslySetInnerHTML={{
          __html: svgString,
        }}
      />
    </>
  );
};

export default SvgViewer;
