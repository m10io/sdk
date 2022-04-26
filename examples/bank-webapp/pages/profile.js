import { Component } from 'react'
import axios from 'axios'
import Page from 'components/page'
import Cookies from 'js-cookie'
import { parseJwt } from 'utils/auth'
import { UserCard } from 'components/info-card'
import routes from 'routes'

class ProfilePage extends Component {
  state = {
    jwtUser: {},
    auth0User: {},
  }

  getUserInfo = async token => {
    const userInfoRes = await axios.get(`${routes.AUTH_API}/user`, {
      headers: { Authorization: `Bearer ${token}` },
    })
    return userInfoRes?.data
  }

  async componentDidMount() {
    try {
      const token = Cookies.get('access_token')
      const jwtUser = parseJwt(token)
      const auth0User = await this.getUserInfo(token)
      this.setState({ jwtUser, auth0User })
    } catch (e) {
      console.log(e)
    }
  }

  render() {
    const {
      jwtUser,
      auth0User,
    } = this.state
    return (
      <Page
        withSidebar
        withGlobalNav
        {...this.props}
      >
        {jwtUser.sub && (
          <UserCard user={{
            ...jwtUser,
            ...auth0User,
          }} />
        )}
      </Page>
    )
  }
}

export default ProfilePage
