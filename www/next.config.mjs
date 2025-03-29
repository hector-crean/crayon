/** @type {import('next').NextConfig} */
const nextConfig = {
    async redirects() {
        return [
          // Basic redirect
          // {
          //   source: '/',
          //   destination: '/files',
          //   permanent: true,
          // },
        ]
      },
};

export default nextConfig;
