import React from 'react';
import MuiGrid, { type GridProps as MuiGridProps } from '@mui/material/Grid';

type Breakpoint = 'xs' | 'sm' | 'md' | 'lg' | 'xl';
type SizeValue = MuiGridProps['size'];

export interface Grid2Props extends Omit<MuiGridProps, 'size'> {
  size?: SizeValue;
  xs?: SizeValue;
  sm?: SizeValue;
  md?: SizeValue;
  lg?: SizeValue;
  xl?: SizeValue;
}

const Grid2 = React.forwardRef<HTMLDivElement, Grid2Props>(
  ({ size, xs, sm, md, lg, xl, ...rest }, ref) => {
    const sizeObject: Record<string, SizeValue> = {};

    if (size && typeof size === 'object') {
      Object.assign(sizeObject, size as Record<string, SizeValue>);
    } else if (size !== undefined) {
      if (typeof size !== 'object') {
          sizeObject.xs = size;
      }
    }

    const responsiveSizes: Array<[Breakpoint, SizeValue | undefined]> = [
      ['xs', xs],
      ['sm', sm],
      ['md', md],
      ['lg', lg],
      ['xl', xl],
    ];

    responsiveSizes.forEach(([breakpoint, value]) => {
      if (value !== undefined) {
        sizeObject[breakpoint] = value;
      }
    });

    const nextProps: MuiGridProps = {
      ...rest,
      size: Object.keys(sizeObject).length > 0 ? sizeObject : size,
    } as MuiGridProps;

    return <MuiGrid ref={ref} {...nextProps} />;
  },
);

Grid2.displayName = 'Grid2';

export default Grid2;
