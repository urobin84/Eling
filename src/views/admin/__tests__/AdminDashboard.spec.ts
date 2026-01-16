
import { describe, it, expect, vi } from 'vitest'
import { mount } from '@vue/test-utils'
import AdminDashboard from '../AdminDashboard.vue'
import { createRouter, createWebHistory } from 'vue-router'

// Mock sub-components
const ScheduleList = { template: '<div>ScheduleList</div>' }
const ManageQuestionsModal = { template: '<div>ManageQuestionsModal</div>' }
const ManageToolsModal = { template: '<div>ManageToolsModal</div>' }
const EventDetailModal = { template: '<div>EventDetailModal</div>' }
const ThemeToggle = { template: '<div>ThemeToggle</div>' }
const NotificationBell = { template: '<div>NotificationBell</div>' }

// Mock router
const router = createRouter({
    history: createWebHistory(),
    routes: [{ path: '/login', component: { template: '<div>Login</div>' } }]
})

describe('AdminDashboard Logout', () => {
    it('opens confirmation dialog when logout button is clicked', async () => {
        localStorage.setItem('user_session', JSON.stringify({ username: 'admin', role: 'admin' }))

        const wrapper = mount(AdminDashboard, {
            global: {
                plugins: [router],
                stubs: {
                    ScheduleList,
                    ManageQuestionsModal,
                    ManageToolsModal,
                    EventDetailModal,
                    ThemeToggle,
                    NotificationBell
                }
            }
        })

        // Mock window.confirm
        const confirmSpy = vi.spyOn(window, 'confirm').mockImplementation(() => true)

        // Mock window.location
        delete (window as any).location
            ; (window as any).location = { href: '' }

        // 1. Open User Dropdown
        const userButton = wrapper.find('[data-testid="user-menu-button"]')
        expect(userButton.exists()).toBe(true)

        // Trigger click to open dropdown
        await userButton.trigger('click')

        // 2. Find Logout Button
        const logoutBtn = wrapper.find('[data-testid="logout-button"]')
        expect(logoutBtn.exists()).toBe(true)

        // 3. Trigger Logout
        await logoutBtn.trigger('click')

        // Wait for setTimeout
        await new Promise(resolve => setTimeout(resolve, 100))

        // 4. Verify Confirmation Dialog was called
        expect(confirmSpy).toHaveBeenCalledWith('Are you sure you want to logout?')

        // 5. Verify redirect happened
        expect(window.location.href).toBe('/login')
    })
})
