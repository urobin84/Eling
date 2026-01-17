import { createRouter, createWebHistory } from 'vue-router';
import LoginView from '../views/auth/LoginView.vue';
import RegisterView from '../views/auth/RegisterView.vue';
import TestRunner from '../views/TestRunner.vue';


const router = createRouter({
    history: createWebHistory(import.meta.env.BASE_URL),
    routes: [
        {
            path: '/',
            name: 'home',
            redirect: '/login'
        },
        {
            path: '/login',
            name: 'login',
            component: LoginView
        },
        {
            path: '/register',
            name: 'register',
            component: RegisterView
        },
        {
            path: '/admin',
            name: 'admin-dashboard',
            component: () => import('../views/admin/AdminDashboard.vue')
        },
        {
            path: '/dashboard', // Candidate default
            name: 'candidate-dashboard',
            component: () => import('../views/candidate/CandidateDashboard.vue')
        },
        {
            path: '/info',
            name: 'app-info',
            component: () => import('../views/candidate/AppInfo.vue')
        },
        {
            path: '/profile',
            name: 'candidate-profile',
            component: () => import('../views/candidate/CandidateProfile.vue')
        },
        {
            path: '/test/run',
            name: 'test-runner',
            component: TestRunner,
            meta: { requiresSurveillance: true },
            beforeEnter: (_to, _from, next) => {
                next();
            }
        },
        {
            path: '/test/kraepelin',
            name: 'kraepelin-test',
            component: () => import('../views/test/KraepelinTest.vue'),
            meta: { requiresSurveillance: true }
        }
    ]
});

// Guard for session persistence
// Guard for session persistence
router.beforeEach((to, from, next) => {
    const sessionStr = localStorage.getItem('user_session');
    console.log(`DEBUG: Router navigation from ${from.path} to ${to.path}`);
    console.log('DEBUG: Current session:', sessionStr);

    // 1. If going to login but already logged in
    /* 
    // DISABLED: User requested to remove Auto-Login (Persistence) behavior
    if (to.path === '/login' && sessionStr) {
        try {
            const session = JSON.parse(sessionStr);
            console.log('DEBUG: User already logged in, redirecting based on role:', session.role);
            if (session.role === 'admin') {
                return next('/admin');
            } else {
                return next('/dashboard');
            }
        } catch (e) {
            console.error('DEBUG: Session parse error, clearing session');
            localStorage.removeItem('user_session'); // Corrupt data
        }
    }
    */

    // 2. Protect Admin Route
    if (to.path.startsWith('/admin')) {
        if (!sessionStr) {
            console.warn('DEBUG: Access denied to /admin (no session)');
            return next('/login');
        }
        try {
            const session = JSON.parse(sessionStr);
            if (session.role !== 'admin') {
                console.warn('DEBUG: Access denied to /admin (wrong role)', session.role);
                return next('/dashboard'); // Candidates shouldn't be here
            }
        } catch (e) {
            return next('/login');
        }
    }

    // 3. Protect Candidate Route
    if (to.path.startsWith('/dashboard') || to.path.startsWith('/test')) {
        if (!sessionStr) {
            console.warn('DEBUG: Access denied to protected route (no session)');
            return next('/login');
        }
    }

    next();
});

export default router;
