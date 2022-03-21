const routes = [
  {
    path: '/profile',
    component: () => import('@/layouts/basic/Index.vue'),
    children: [
      {
        path: '',
        name: 'Profile Page',
        component: () => import('@/views/profile/Index.vue')
      }
    ]
  }
]

export default routes
