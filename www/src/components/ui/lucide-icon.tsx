import { icons, type LucideProps } from 'lucide-react';

type IconName = keyof typeof icons

interface IconProps extends LucideProps {
  name: IconName

};

const LucideIcon = ({ name, color, size }: IconProps) => {
  const LucideIcon = icons[name];

  return <LucideIcon color={color} size={size} />;
};

export { LucideIcon }
export type { IconName }